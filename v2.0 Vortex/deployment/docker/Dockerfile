# Multi-stage build for optimized image size
FROM gcc:12 AS builder

# Install build dependencies
RUN apt-get update && apt-get install -y \
    cmake \
    ninja-build \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /build

# Copy source files
COPY CMakeLists.txt ./
COPY src/ ./src/
COPY include/ ./include/
COPY tools/ ./tools/

# Build with security hardening
RUN cmake -B build -G Ninja \
    -DCMAKE_BUILD_TYPE=Release \
    -DCMAKE_INSTALL_PREFIX=/opt/hypercycle \
    -DHC_BUILD_TESTS=OFF \
    && cmake --build build --parallel \
    && cmake --install build

# Production stage - minimal runtime image
FROM debian:12-slim

# Install runtime dependencies only
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -u 1000 -g users hypercycle

# Copy built artifacts from builder
COPY --from=builder /opt/hypercycle /opt/hypercycle

# Set ownership
RUN chown -R hypercycle:users /opt/hypercycle

# Switch to non-root user
USER hypercycle

# Set working directory
WORKDIR /opt/hypercycle

# Expose ports (if needed for future API)
# EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD /opt/hypercycle/bin/vortex_dashboard --health || exit 1

# Default command
CMD ["/opt/hypercycle/bin/vortex_dashboard"]

# Labels
LABEL org.opencontainers.image.title="HyperCycle Vortex"
LABEL org.opencontainers.image.description="Post-Quantum Cryptographic Entropy Engine"
LABEL org.opencontainers.image.version="2.0.2"
LABEL org.opencontainers.image.vendor="HyperCycle"
LABEL org.opencontainers.image.licenses="Proprietary"
LABEL org.opencontainers.image.source="https://github.com/hypercycle/vortex"
