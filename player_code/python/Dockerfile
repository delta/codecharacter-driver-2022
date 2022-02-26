FROM gcr.io/distroless/python3-debian11

ENV PYTHONUNBUFFERED=x

WORKDIR /player_code

COPY main.py player_code.py ./

CMD [ "-u", "main.py" ]
