import { hostname } from 'os';

export default defineEventHandler(() => {
  return {
    hostname: hostname(),
  };
});
