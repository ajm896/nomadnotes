# Step 1: Use an official Node.js image as the builder
FROM node:latest AS builder

WORKDIR /app

# Copy package.json and install dependencies
COPY package.json ./
RUN npm install 

# Copy the rest of the project files
COPY . .

# Install the correct version of esbuild
RUN npm install esbuild@0.25.0

# Build the project for production
RUN npm run build

# Step 2: Use an Nginx image to serve the built frontend
FROM nginx:latest

# Copy the built files to Nginx's web root
COPY --from=builder /app/dist /usr/share/nginx/html

# Expose port 80
EXPOSE 80

# Start Nginx
CMD ["nginx", "-g", "daemon off;"]