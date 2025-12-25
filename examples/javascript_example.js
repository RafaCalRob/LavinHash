// JavaScript Example for LavinHash
// Works with modern bundlers (Vite, Webpack, Rollup, etc.)

import {
  wasm_generate_hash,
  wasm_compare_hashes,
  wasm_compare_data
} from '../pkg/lavinhash.js';

console.log('LavinHash JavaScript Example\n');

// Example 1: Compare two similar texts directly
console.log('=== Example 1: Direct Comparison ===');
const encoder = new TextEncoder();

const text1 = encoder.encode("The quick brown fox jumps over the lazy dog");
const text2 = encoder.encode("The quick brown fox leaps over the lazy dog");

try {
  const similarity = wasm_compare_data(text1, text2);
  console.log('Text 1:', new TextDecoder().decode(text1));
  console.log('Text 2:', new TextDecoder().decode(text2));
  console.log('Similarity:', similarity + '%\n');
} catch (error) {
  console.error('Error:', error);
}

// Example 2: Generate hashes and compare separately
console.log('=== Example 2: Generate and Compare ===');

const data1 = encoder.encode("Hello, World! This is a test.");
const data2 = encoder.encode("Hello, World! This is a test.");

try {
  const hash1 = wasm_generate_hash(data1);
  const hash2 = wasm_generate_hash(data2);

  console.log('Hash 1 size:', hash1.length, 'bytes');
  console.log('Hash 2 size:', hash2.length, 'bytes');

  const similarity = wasm_compare_hashes(hash1, hash2);
  console.log('Similarity (should be 100%):', similarity + '%\n');
} catch (error) {
  console.error('Error:', error);
}

// Example 3: Compare different texts
console.log('=== Example 3: Different Texts ===');

const dataA = encoder.encode("Completely different content");
const dataB = encoder.encode("ZZZZZZZZZZZZZZZZZZZZZZZZZZZZ");

try {
  const similarity = wasm_compare_data(dataA, dataB);
  console.log('Text A:', new TextDecoder().decode(dataA));
  console.log('Text B:', new TextDecoder().decode(dataB));
  console.log('Similarity (should be low):', similarity + '%\n');
} catch (error) {
  console.error('Error:', error);
}

// Example 4: File comparison in web apps
console.log('=== Example 4: File Comparison ===');
console.log(`
// In a React/Vue/Angular app with file upload:
import { wasm_compare_data } from 'lavinhash';

async function handleFileUpload(file1, file2) {
  const buffer1 = await file1.arrayBuffer();
  const buffer2 = await file2.arrayBuffer();

  const data1 = new Uint8Array(buffer1);
  const data2 = new Uint8Array(buffer2);

  const similarity = wasm_compare_data(data1, data2);
  console.log('Files similarity:', similarity + '%');
}
`);

// Example 5: Batch processing
console.log('=== Example 5: Batch Processing ===');

const documents = [
  "Document 1: The quick brown fox",
  "Document 2: The quick brown dog",
  "Document 3: A completely different text",
  "Document 4: The quick brown fox jumps"
];

try {
  const hashes = documents.map(doc =>
    wasm_generate_hash(encoder.encode(doc))
  );

  console.log('Generated', hashes.length, 'hashes\n');
  console.log('Pairwise similarities:');

  for (let i = 0; i < hashes.length; i++) {
    for (let j = i + 1; j < hashes.length; j++) {
      const sim = wasm_compare_hashes(hashes[i], hashes[j]);
      console.log(`Doc ${i+1} vs Doc ${j+1}: ${sim}%`);
    }
  }
} catch (error) {
  console.error('Error:', error);
}
