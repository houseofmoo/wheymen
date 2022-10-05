# launch command
# docker run --p <HOST PORT>:<CONTAINER PORT> --network <NETWORK NAME> --name <CONTAINER NAME> <IMAGE NAME>
# docker run -p 8080:8080 server

# build image
# docker build -t <NAME> -f <DOCKER FILE NAME> .

# Create user defined network:
# docker network create <NETWORK NAME>

# Delete user defined network:
# docker network rm <NETWORK NAME>

# List networks
# docker network ls

# Start an image on the user defined network
# --network <NETWORK NAME>

# Connect host:container ports
# --publish HOST_PORT:CONTAINER_PORT

# Connect already running container to a existing user defined network
# docker network connect <NETWORK NAME> <CONTAINER NAME>

# Disconnect container from user defined network
# docker network disconnect <NETWORK NAME> <CONTAINER NAME>