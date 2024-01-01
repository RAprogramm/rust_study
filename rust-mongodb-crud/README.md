<h1>Rust MongoDB CRUD</h1>

<p>This project is a Rust-based API service using the Warp framework, providing endpoints for managing notes.</p>

<h2>Table of Contents</h2>

<ul>
  <li><a href="#overview">Overview</a></li>
  <li><a href="#requirements">Requirements</a></li>
  <li><a href="#getting-started">Getting Started</a></li>
  <li><a href="#configuration">Configuration</a></li>
  <li><a href="#usage">Usage</a></li>
</ul>

<h2 id="overview">Overview</h2>

<p>The project implements a RESTful API service for managing notes. It uses the Warp framework in Rust and connects to a MongoDB database.</p>

<h2 id="requirements">Requirements</h2>

<ul>
  <li>Rust</li>
  <li>Docker</li>
</ul>

<h2 id="getting-started">Getting Started</h2>

<h3>Running the Application</h3>

> I'll show you how to do it with terminal command in linux.

1. [Download the repository](https://downgit.evecalm.com/#/home?url=https://github.com/RAprogramm/rust_study/tree/main/rust-mongodb-crud).

2. Go to your download folder.

   > for example
   >
   > ```sh
   > cd  ~/Downloads
   > ```

3. Extract project from ZIP archive.

   > in Downloads folder
   >
   > ```sh
   > unzip rust-mongodb-crud.zip
   > ```

4. Enter the project.

   > after extracting
   >
   > ```sh
   > cd rust-mongodb-crud
   > ```

5. Start MongoDB using Docker:

```sh
make mongo_in_docker
```

6. Build and run the project:

```sh
make dev
```

<h3>Configuration</h3>

<ul>
  <li>The project uses environment variables for configuration.</li>
</ul>

<h2 id="usage">Usage</h2>

<p>Access the API at <code>http://localhost:8080/api/notes</code></p>
<p>Check heath of the API at <code>http://localhost:8080/api/healthchecker</code></p>
