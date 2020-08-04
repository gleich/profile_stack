FROM python:3.8.5-slim-buster

COPY . /app
WORKDIR /app

# Installing dependencies:
RUN python -m pip install --upgrade pip
RUN pip3 install poetry==1.0.10
RUN poetry config virtualenvs.create false
RUN poetry install --no-root --no-dev -n

# Installing git
RUN apt-get update && apt-get install git=2.20.1 -y --no-install-recommends \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*
