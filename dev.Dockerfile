FROM python:3.8.5-slim-buster

COPY . /app
WORKDIR /app

# Installing dependencies:
RUN python -m pip install --upgrade pip
RUN pip install poetry==1.0.10
RUN poetry config virtualenvs.create false
RUN poetry install --no-root -n

# Installing git
RUN apt-get update && apt-get install git=1:2.20.1-2+deb10u3 -y --no-install-recommends \
    && apt-get clean \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app/tests

CMD [ "pytest", "-vv" ]
