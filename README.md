# Links

## Development

```bash
PORT=3000 cargo run
```

## Docker

Build the image and expose port 3000:

```bash
docker build -t links .
docker run --rm -p 3000:3000 links
```

```bash
docker tag links your-registry/links
docker push your-registry/links
```
