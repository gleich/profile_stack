FROM python:3.8

COPY . /app
WORKDIR /app

RUN pip3 install poetry
RUN poetry config virtualenvs.create false
RUN poetry install --no-root -n

WORKDIR /app/tests
CMD [ "pytest", "-vv" ]
