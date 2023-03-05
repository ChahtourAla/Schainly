FROM node:16.19.1-bullseye

WORKDIR /app

COPY package.json yarn.lock /app/

RUN yarn 

COPY . .

CMD ["yarn", "start"]