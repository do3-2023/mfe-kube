import { useLogger } from '@nuxt/kit';

const config = useRuntimeConfig();
const logger = useLogger();

export default defineEventHandler(async (event) => {
  const body = await readBody(event);
  const data = await $fetch(`${config.public.apiBase}/messages`, { method: 'POST', body })
    .then((res) => {
      logger.info(`${new Date().toISOString()} | INFO method=POST uri=${config.public.apiBase}/messages body="${JSON.stringify(body)}"`);

      return res;
    })
    .catch((e) => {
      logger.error(`${new Date().toISOString()} | ERROR method=POST uri=${config.public.apiBase}/messages`, e);

      return null;
    });

  return data;
});
