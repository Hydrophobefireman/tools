_default:
  just --list

docker:
    #!/bin/bash -eux
    nix build .#dockerImage
    docker load < ./result

deploy:
    just docker
    fly deploy --local-only
    just clean

frontend:
    #!/bin/bash -eux
    cd frontend
    npm run build

clean:
    rm -rf result