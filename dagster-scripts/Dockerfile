FROM python:3.10-slim

ARG GEMFURY_TOKEN=${GEMFURY_TOKEN}

RUN apt update && apt upgrade --yes && apt install libpq-dev git build-essential wget --yes

WORKDIR /scripts

COPY ./requirements.txt requirements.txt

RUN GEMFURY_TOKEN=$GEMFURY_TOKEN pip3 install -r requirements.txt

COPY . .

RUN sed -i '/^.*#/d' workspace.yaml

ENV PYTHONPATH "${PYTHONPATH}:/scripts"

CMD [ "tail", "-f", "/dev/null" ]
