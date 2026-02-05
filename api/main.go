package main

import (
	"log"
	"net/http"
	"os"
	"strconv"
	"time"

	"github.com/gin-contrib/cors"
	"github.com/gin-gonic/gin"
	"github.com/meilisearch/meilisearch-go"
	"golang.org/x/time/rate"
)

type SearchRequest struct {
	Query  string `json:"query" binding:"required"`
	Limit  int    `json:"limit"`
	Offset int    `json:"offset"`
}

type SearchResponse struct {
	Query      string         `json:"query"`
	Total      int64          `json:"total"`
	Results    []SearchResult `json:"results"`
	SearchTime int64          `json:"search_time_ms"`
}

type SearchResult struct {
	Title   string  `json:"title"`
	URL     string  `json:"url"`
	Snippet string  `json:"snippet"`
	Score   float64 `json:"score"`
}

type StatsResponse struct {
	TotalIndexed int64  `json:"total_indexed"`
	IndexSizeMB  int64  `json:"index_size_mb"`
	LastCrawl    string `json:"last_crawl"`
}

var (
	meiliClient *meilisearch.Client
	limiter     *rate.Limiter
)

func main() {
	// Get environment variables
	meilisearchURL := os.Getenv("MEILISEARCH_URL")
	if meilisearchURL == "" {
		meilisearchURL = "http://localhost:7700"
	}

	meilisearchKey := os.Getenv("MEILISEARCH_KEY")
	if meilisearchKey == "" {
		meilisearchKey = os.Getenv("MEILI_MASTER_KEY")
	}

	if meilisearchKey == "" {
		log.Fatal("MEILISEARCH_KEY or MEILI_MASTER_KEY must be set")
	}

	port := os.Getenv("PORT")
	if port == "" {
		port = "8080"
	}

	rateLimit, _ := strconv.Atoi(os.Getenv("RATE_LIMIT"))
	if rateLimit == 0 {
		rateLimit = 100
	}

	// Initialize Meilisearch client
	meiliClient = meilisearch.NewClient(meilisearch.ClientConfig{
		Host:   meilisearchURL,
		APIKey: meilisearchKey,
	})

	// Initialize rate limiter (requests per minute)
	limiter = rate.NewLimiter(rate.Limit(rateLimit)/60, rateLimit)

	// Setup Gin
	if os.Getenv("GIN_MODE") == "release" {
		gin.SetMode(gin.ReleaseMode)
	}

	r := gin.Default()

	// CORS middleware
	r.Use(cors.New(cors.Config{
		AllowOrigins:     []string{"*"},
		AllowMethods:     []string{"GET", "POST", "OPTIONS"},
		AllowHeaders:     []string{"Origin", "Content-Type", "Accept"},
		ExposeHeaders:    []string{"Content-Length"},
		AllowCredentials: false,
		MaxAge:           12 * time.Hour,
	}))

	// Rate limiting middleware
	r.Use(rateLimitMiddleware())

	// Routes
	r.GET("/", handleRoot)
	r.GET("/health", handleHealth)
	r.GET("/stats", handleStats)
	r.POST("/search", handleSearch)

	// Start server
	log.Printf("Starting API server on port %s", port)
	if err := r.Run(":" + port); err != nil {
		log.Fatal("Failed to start server:", err)
	}
}

func rateLimitMiddleware() gin.HandlerFunc {
	return func(c *gin.Context) {
		if !limiter.Allow() {
			c.JSON(http.StatusTooManyRequests, gin.H{
				"error": "Rate limit exceeded. Please try again later.",
			})
			c.Abort()
			return
		}
		c.Next()
	}
}

func handleRoot(c *gin.Context) {
	c.JSON(http.StatusOK, gin.H{
		"message": "Search Engine API",
		"version": "1.0.0",
		"status":  "running",
		"endpoints": gin.H{
			"search": "POST /search",
			"health": "GET /health",
			"stats":  "GET /stats",
		},
	})
}

func handleHealth(c *gin.Context) {
	// Check Meilisearch connection
	healthy := true
	_, err := meiliClient.Health()
	if err != nil {
		healthy = false
	}

	status := "healthy"
	if !healthy {
		status = "unhealthy"
	}

	c.JSON(http.StatusOK, gin.H{
		"status":      status,
		"meilisearch": healthy,
		"timestamp":   time.Now().Unix(),
	})
}

func handleStats(c *gin.Context) {
	index := meiliClient.Index("web_pages")

	stats, err := index.GetStats()
	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": "Failed to get stats: " + err.Error(),
		})
		return
	}

	c.JSON(http.StatusOK, gin.H{
		"total_indexed": stats.NumberOfDocuments,
		"is_indexing":   stats.IsIndexing,
		"last_crawl":    time.Now().Format(time.RFC3339),
	})
}

func handleSearch(c *gin.Context) {
	var req SearchRequest
	if err := c.ShouldBindJSON(&req); err != nil {
		c.JSON(http.StatusBadRequest, gin.H{
			"error": "Invalid request: " + err.Error(),
		})
		return
	}

	// Set defaults
	if req.Limit == 0 {
		req.Limit = 20
	}
	if req.Limit > 100 {
		req.Limit = 100
	}

	// Search Meilisearch
	startTime := time.Now()

	index := meiliClient.Index("web_pages")
	searchRes, err := index.Search(req.Query, &meilisearch.SearchRequest{
		Limit:  int64(req.Limit),
		Offset: int64(req.Offset),
	})

	if err != nil {
		c.JSON(http.StatusInternalServerError, gin.H{
			"error": "Search failed: " + err.Error(),
		})
		return
	}

	searchTime := time.Since(startTime).Milliseconds()

	// Format results
	results := make([]SearchResult, 0)
	for _, hit := range searchRes.Hits {
		hitMap := hit.(map[string]interface{})

		result := SearchResult{
			Title:   getStringField(hitMap, "title"),
			URL:     getStringField(hitMap, "url"),
			Snippet: getStringField(hitMap, "snippet"),
			Score:   1.0, // Meilisearch doesn't expose scores directly
		}
		results = append(results, result)
	}

	response := SearchResponse{
		Query:      req.Query,
		Total:      searchRes.EstimatedTotalHits,
		Results:    results,
		SearchTime: searchTime,
	}

	c.JSON(http.StatusOK, response)
}

func getStringField(m map[string]interface{}, key string) string {
	if val, ok := m[key]; ok {
		if str, ok := val.(string); ok {
			return str
		}
	}
	return ""
}
