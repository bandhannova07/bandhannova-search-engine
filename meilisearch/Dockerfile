FROM getmeili/meilisearch:v1.6

# Environment variables
ENV MEILI_ENV=production
ENV MEILI_NO_ANALYTICS=true
ENV MEILI_HTTP_ADDR=0.0.0.0:7700

# Expose port
EXPOSE 7700

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=10s --retries=3 \
    CMD curl -f http://localhost:7700/health || exit 1
