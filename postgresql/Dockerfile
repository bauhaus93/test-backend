FROM postgres

EXPOSE 5432

COPY --chown=postgres:postgres init/ /docker-entrypoint/
COPY --chown=postgres:postgres config/ /var/lib/postgresql/initial_config/
