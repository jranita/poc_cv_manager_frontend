# poc_cv_manager_frontend

Frontend for the OpenAPI poc_cv_manager backend
Technologies:
Rust, Dioxus, Tailwindcss, Docker

run:
docker-compose up

The image includes the command to compile Rust code automaticaly.
But for Tailwindcss the --watch does not work very well, you will need a bash session inside your docker container:
docker-compose exec app bash

In the bash terminal to start the Dev Server run:
dx serve --hot-reload &

To generate the tailwind css in the docker container terminal run:
npx tailwindcss -i ./input.css -o ./dist/output.css

If you are not making changes you can skip the two previous paragraphs and do:
docker-compose exec app dx serve --hot-reload

Open the browser on http://localhost:8080
