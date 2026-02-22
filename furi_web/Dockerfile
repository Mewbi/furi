FROM node:20-alpine AS build
COPY . /app
WORKDIR /app
ENV VITE_API_URL=furi.local/
ENV VITE_API_ANALYTICS_URL=analytics.furi.local/
RUN yarn install --frozen-lockfile && yarn build 

FROM alpine:latest
RUN apk add --no-cache rsync
COPY --from=build /app/dist /app
