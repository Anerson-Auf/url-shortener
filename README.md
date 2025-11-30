# URL Shortener

Веб-приложение для сокращения URL на Rust.

![Interface](https://u.yufu.su/vdJ.png)

## Требования

- Rust
- PostgreSQL
- Redis

## Установка

```bash
cargo build --release
```

## Запуск

Создайте `.env` файл:

```env
DATABASE_URL=postgresql://user:password@localhost/dbname
REDIS_URL=redis://localhost:6379
IP=127.0.0.1
PORT=8080
BASE_URL=http://127.0.0.1:8080
```

Запустите:

```bash
cargo run
```

## API

- `POST /api/short-url` - создание короткой ссылки
- `GET /api/config` - получение конфигурации
- `GET /r/{code}` - редирект по короткой ссылке

## Деплой

Установите `BASE_URL=https://yourdomain.com` в `.env`.

