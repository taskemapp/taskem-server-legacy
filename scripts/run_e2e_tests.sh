docker compose down --volumes && docker compose build --no-cache && docker compose up -d &&
    cargo test --test auth_tests &&
docker compose down --volumes
