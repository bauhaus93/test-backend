FROM postgres

EXPOSE 5432
ENV POSTGRES_USER postgres
ENV POSTGRES_PASSWORD secret
ENV POSTGRES_DB backend-test

COPY --chown=postgres:postgres setup/*.conf /var/lib/postgresql/initial_config/
COPY --chown=postgres:postgres setup/*.sh setup/*.sql /docker-entrypoint-initdb.d/
