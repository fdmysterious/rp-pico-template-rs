image_name := env_var_or_default("IMAGE_NAME", "pico-env")

# https://stackoverflow.com/questions/23513045/how-to-check-if-a-process-is-running-inside-docker-container
is_in_docker := `test -f /.dockerenv && echo 1 || echo 0`

# run_cmd is the docker run command used to launch your env.
run_cmd := if is_in_docker == "0" {
    'docker run -it --rm --mount type=bind,src="$(pwd)",target="/project" -w /project ' + image_name
} else {""}

#################

echo-run-cmd:
    @echo run_cmd={{run_cmd}}

check-in-docker:
    @echo {{if is_in_docker == "1" {"In docker container!"} else  {"Not in docker container!"} }}

ensure-folders:
    @mkdir -p build-tests
    @mkdir -p logs

build-docker force="no": ensure-folders
    #!/usr/bin/env sh
    if [ {{is_in_docker}} -eq 1 ] ; then
        echo "Running in docker container, skipping image generation !"
    else
        if [ -z `docker image ls {{image_name}} --format '1'`] || [ {{force}} = "force" ] ; then
            [ {{force}} = "force" ] && echo "Forcing build"
            docker buildx build --load -t {{image_name}} docker
        fi
    fi



#################

build: build-docker
    {{run_cmd}} cargo build --release

shell: build-docker
    {{run_cmd}} /bin/bash

cargo +args: build-docker
    {{run_cmd}} cargo {{args}}
	
# TODO # Make this compatible outside of docker container
get-uf2: build
    {{run_cmd}} /home/builder/.cargo/bin/elf2uf2-rs target/thumbv6m-none-eabi/release/rp-pico-template rp-pico-template.uf2
