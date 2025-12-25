<!-- Vue 3 Example for LavinHash -->
<!-- Works with Vite, Vue CLI, Nuxt 3, etc. -->

<template>
  <div class="similarity-checker">
    <h2>Text Similarity Checker</h2>

    <div>
      <label>Text 1:</label>
      <textarea
        v-model="text1"
        @input="calculateSimilarity"
        rows="3"
      ></textarea>
    </div>

    <div>
      <label>Text 2:</label>
      <textarea
        v-model="text2"
        @input="calculateSimilarity"
        rows="3"
      ></textarea>
    </div>

    <div>
      <h3>Similarity: {{ similarity }}%</h3>
    </div>

    <button @click="calculateSimilarity">Recalculate</button>
  </div>
</template>

<script setup>
import { ref, onMounted, watch } from 'vue';
import { wasm_compare_data, wasm_generate_hash } from 'lavinhash';

const text1 = ref('The quick brown fox jumps over the lazy dog');
const text2 = ref('The quick brown fox leaps over the lazy dog');
const similarity = ref(null);

const calculateSimilarity = () => {
  const encoder = new TextEncoder();
  const data1 = encoder.encode(text1.value);
  const data2 = encoder.encode(text2.value);

  similarity.value = wasm_compare_data(data1, data2);
};

onMounted(() => {
  calculateSimilarity();
});

// Example: Compare files
const compareFiles = async (file1, file2) => {
  const buffer1 = await file1.arrayBuffer();
  const buffer2 = await file2.arrayBuffer();

  const data1 = new Uint8Array(buffer1);
  const data2 = new Uint8Array(buffer2);

  return wasm_compare_data(data1, data2);
};
</script>
