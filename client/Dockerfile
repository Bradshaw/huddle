FROM node:20 as build-stage

WORKDIR /usr/src/project

COPY package*.json ./

RUN npm ci

COPY tsconfig.json .
COPY public ./public

COPY src ./src

RUN npm run build

FROM nginx:stable-alpine as production-stage
ARG PORT=80
ARG SERVER_ADDRESS=http://server:3001/

COPY nginx.conf.template /etc/nginx/nginx.conf.template
RUN envsubst '{{$PORT $SERVER_ADDRESS}}' < /etc/nginx/nginx.conf.template > /etc/nginx/nginx.conf

COPY --from=build-stage /usr/src/project/build /usr/share/nginx/html

EXPOSE 80