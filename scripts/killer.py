from time import sleep, time
import docker
from datetime import datetime
import docker, dateparser

client = docker.from_env()


def compare_time(container_started_at):
    dt = dateparser.parse(container_started_at).timestamp()
    # get currrent timestamp
    now = time()
    interval = now - dt
    return interval


tags = [
    "ghcr.io/delta/codecharacter-cpp-compiler",
    "ghcr.io/delta/codecharacter-cpp-runner",
    "ghcr.io/delta/codecharacter-java-compiler",
    "ghcr.io/delta/codecharacter-java-runner",
    "ghcr.io/delta/codecharacter-python-runner",
    "ghcr.io/delta/codecharacter-simulator",
]


def check_containers():
    print("Checking containers now at", time())
    for c in client.containers.list():
        # tag of the container
        tag = c.image.tags[0].split(":")[0]
        start_at = c.attrs["State"]["StartedAt"]
        interval = compare_time(start_at)
        deleteable = False
        if tag in tags and interval > 15:

            try:
                print(
                    c.name + " (" + c.id + ") " + str(c.image) + " " + str(deleteable)
                )
                client.containers.get(c.id).kill()
                print("Deleted: " + c.name)
            except docker.errors.NotFound:
                print("Not Found ", c)
                pass
        else:
            continue


while True:
    check_containers()
    sleep(15)
