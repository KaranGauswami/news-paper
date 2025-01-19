# Use Debian 12 slim as the base image
FROM debian:12-slim

# Set environment variables for non-interactive installation
ENV DEBIAN_FRONTEND=noninteractive

# Install dependencies, Geckodriver, and Firefox
RUN apt-get update && apt-get install -y wget firefox-esr

# Install Geckodriver
RUN wget -q https://github.com/mozilla/geckodriver/releases/download/v0.35.0/geckodriver-v0.35.0-linux-aarch64.tar.gz \
    && tar -xzf geckodriver-v0.35.0-linux-aarch64.tar.gz \
    && mv geckodriver /usr/local/bin/ \
    && rm geckodriver-v0.35.0-linux-aarch64.tar.gz

# Set the working directory
WORKDIR /app

# Copy your binary from the current folder to the container
COPY target/release/news-paper-scrapper /app/

# Make the binary executable
RUN chmod +x /app/news-paper-scrapper

COPY --chmod=755 <<EOT /entrypoint.sh
#!/usr/bin/env bash
set -e
geckodriver --port 4444 > /dev/null 2>&1 &
geckodriver --port 4445 > /dev/null 2>&1 &
/app/news-paper-scrapper
EOT

ENTRYPOINT ["/entrypoint.sh"]