FROM nginx:1.21.3-alpine

COPY ./wait-for-backend.sh /wait-for-backend.sh
COPY nginx.conf /etc/nginx/nginx.conf

WORKDIR /

RUN apk add --no-cache dos2unix && \
    dos2unix /wait-for-backend.sh && \
    chmod +x /wait-for-backend.sh && \
    apk del dos2unix

CMD ["./wait-for-backend.sh"]