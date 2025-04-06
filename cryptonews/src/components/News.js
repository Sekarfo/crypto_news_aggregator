import React, { useState } from "react";
import axios from "axios";

const News = () => {
  const [query, setQuery] = useState("");
  const [articles, setArticles] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState("");

  const fetchNews = async () => {
    setLoading(true);
    setError("");

    try {
      const response = await axios.get(`http://localhost:8080/news/${query}`);
      setArticles(response.data);
    } catch (err) {
      setError("Failed to fetch news. Please try again.");
    } finally {
      setLoading(false);
    }
  };

  return (
    <div style={{ padding: "20px" }}>
      <h1>Crypto News Aggregator</h1>
      <div>
        <input
          type="text"
          placeholder="Enter a keyword (e.g., bitcoin)"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
          style={{ padding: "10px", width: "300px" }}
        />
        <button onClick={fetchNews} style={{ padding: "10px 20px", marginLeft: "10px" }}>
          Search
        </button>
      </div>
      {loading && <p>Loading...</p>}
      {error && <p style={{ color: "red" }}>{error}</p>}
      <div style={{ marginTop: "20px" }}>
        {articles.map((article, index) => (
          <div key={index} style={{ marginBottom: "20px" }}>
            <h3>{article.title}</h3>
            <p>
              <strong>Source:</strong> {article.source}
            </p>
            <p>
              <strong>Published At:</strong> {article.published_at}
            </p>
            <p>{article.summary}</p>
            <a href={article.url} target="_blank" rel="noopener noreferrer">
              Read more
            </a>
          </div>
        ))}
      </div>
    </div>
  );
};

export default News;