FROM node:20-alpine as builder

WORKDIR /app
COPY package*.json ./

RUN npm install --legacy-peer-deps

COPY . .
COPY .env .env
RUN npm run build

FROM caddy:2.6.4-alpine

WORKDIR /site

COPY --from=builder /app/build .

# Expose port 80
EXPOSE 80
COPY Caddyfile Caddyfile
# Start the Caddy server
CMD ["caddy", "run","--config=/site/Caddyfile"]