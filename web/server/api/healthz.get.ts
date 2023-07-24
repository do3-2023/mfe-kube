import { useLogger } from '@nuxt/kit';

const config = useRuntimeConfig();
const logger = useLogger();

export default defineEventHandler(async () => {
  const data = await $fetch<string>(`${config.public.apiBase}/healthz`, { method: 'GET' })
    .then((res) => {
      logger.info(`${new Date().toISOString()} | INFO is API healthy? ->`, res);

      return res;
    })
    .catch((e) => {
      logger.error(`${new Date().toISOString()} | ERROR is API healthy? ->`, e.data ?? "couldn't reach the API");

      throw createError({
        statusCode: 500,
        statusMessage: 'not healthy',
        message: e.data ? '' : "couldn't reach the API",
        stack: '',
      });
    });

  return {
    statusCode: 200,
    statusMessage: data,
  };
});
