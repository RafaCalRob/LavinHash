// React Example for LavinHash
// Works with Vite, Create React App, Next.js, etc.

import { useState, useEffect } from 'react';
import { wasm_compare_data, wasm_generate_hash } from 'lavinhash';

function TextSimilarityChecker() {
  const [text1, setText1] = useState('The quick brown fox jumps over the lazy dog');
  const [text2, setText2] = useState('The quick brown fox leaps over the lazy dog');
  const [similarity, setSimilarity] = useState(null);

  const calculateSimilarity = () => {
    const encoder = new TextEncoder();
    const data1 = encoder.encode(text1);
    const data2 = encoder.encode(text2);

    const result = wasm_compare_data(data1, data2);
    setSimilarity(result);
  };

  useEffect(() => {
    calculateSimilarity();
  }, [text1, text2]);

  return (
    <div>
      <h2>Text Similarity Checker</h2>

      <div>
        <label>Text 1:</label>
        <textarea
          value={text1}
          onChange={(e) => setText1(e.target.value)}
          rows={3}
        />
      </div>

      <div>
        <label>Text 2:</label>
        <textarea
          value={text2}
          onChange={(e) => setText2(e.target.value)}
          rows={3}
        />
      </div>

      <div>
        <h3>Similarity: {similarity}%</h3>
      </div>

      <button onClick={calculateSimilarity}>Recalculate</button>
    </div>
  );
}

export default TextSimilarityChecker;
