trap 'docker kill $(docker ps -q -f ancestor=proxy)' 2

docker build -t proxy .
docker run --rm -d -p 80:80 proxy:latest
sleep 99999
