<template>
  <textarea
    v-model.trim="inputMessage"
    autocomplete="off"
    class="textarea grow rounded-r-sm h-0 max-h-40 focus:outline-none"
    placeholder="Press enter to send a message"
    @keyup="handleReturnKey"
  ></textarea>
</template>

<script setup lang="ts">
const inputMessage = ref('');

const handleReturnKey = (e: KeyboardEvent) => {
  if (e.getModifierState('Shift') || e.key !== 'Enter') return;

  sendMessage();
};

const sendMessage = async () => {
  if (inputMessage.value === '') return;

  const { status } = await useFetch('/api/messages', {
    method: 'POST',
    body: {
      content: inputMessage.value,
    },
  });

  if (status.value === 'error') return;

  inputMessage.value = '';
};
</script>

<style scoped>
.textarea {
  background-color: var(--fg-color);
}
</style>
