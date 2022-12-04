FROM nginx:1.23.2
LABEL description="Api Gateway for Microservice Fuhrparkverwaltung with NGINX"
RUN rm /etc/nginx/conf.d/default.conf
COPY ./config/nginx.conf /etc/nginx/conf.d/default.conf