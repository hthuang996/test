FROM node:14.15.0

# Create directory
WORKDIR /opt/dante/node/

# Move source files to docker image
COPY . .

# Install dependencies
RUN npm install

# Run
ENTRYPOINT cat config/default.json