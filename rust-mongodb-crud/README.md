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

<ol>
  <li>Clone the repository:
    <pre><code>git clone https://github.com/RAprogramm/rust-study/rust-mongodb-crud.git
    cd rust-study/rust-mongodb-crud</code></pre>
  </li>
  <li>Start MongoDB using Docker:
    <pre><code>make mongo_in_docker</code></pre>
  </li>
  <li>Build and run the project:
    <pre><code>make dev</code></pre>
  </li>
</ol>

<h3>Configuration</h3>

<ul>
  <li>The project uses environment variables for configuration.</li>
</ul>

<h2 id="usage">Usage</h2>

<p>Access the API at <code>http://localhost:8080/api/notes</code></p>
