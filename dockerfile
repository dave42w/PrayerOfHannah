#FROM python:3.12-slim-bookworm AS base
#
#FROM base AS builder
#COPY --from=ghcr.io/astral-sh/uv:0.5.11 /uv /bin/uv
#ENV UV_COMPILE_BYTECODE=1 UV_LINK_MODE=copy
#WORKDIR /app
#COPY uv.lock pyproject.toml /app/
#RUN --mount=type=cache,target=/root/.cache/uv \
#  uv sync --frozen --no-install-project --no-dev
#COPY . /app
#RUN --mount=type=cache,target=/root/.cache/uv \
#  uv sync --frozen --no-dev
#
#
FROM base
COPY --from=builder . /app
ENV PATH="/app/.venv/bin:$PATH"
EXPOSE 8080

CMD ["python", "src/prayerofhannah/poh.py"]

FROM ghcr.io/astral-sh/uv:python3.12-bookworm-slim AS builder

ENV UV_COMPILE_BYTECODE=1 UV_LINK_MODE=copy

WORKDIR /app

RUN --mount=type=cache,target=/root/.cache/uv \
  --mount=type=bind,source=uv.lock,target=uv.lock \
  --mount=type=bind,source=pyproject.toml,target=pyproject.toml \
  uv sync --frozen --no-install-project --no-dev
COPY . /app
RUN --mount=type=cache,target=/root/.cache/uv \
  uv sync --frozen --no-dev

# Then, use a final image without uv
FROM python:3.12-slim-bookworm
# It is important to use the image that matches the builder, as the path to the
# Python executable must be the same, e.g., using `python:3.11-slim-bookworm`
# will fail.

# Copy the application from the builder
COPY --from=builder --chown=app:app /app /app

# Place executables in the environment at the front of the path
ENV PATH="/app/.venv/bin:$PATH"
EXPOSE 8080
CMD ["/app/.venv/bin/robyn", "app/src/prayerofhannah/poh.py"]