FROM nginx:1.21.3-alpine

COPY wait-for-backend.sh /wait-for-backend.sh
COPY nginx.conf /etc/nginx/nginx.conf

RUN chmod +x /wait-for-backend.sh

CMD ["/wait-for-backend.sh"]