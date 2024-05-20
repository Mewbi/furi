import axios from 'axios';

type ShortUrlResponse = {
  original_url: string;
  short_url: string;
};

const API_URL = import.meta.env.VITE_API_URL;

const apiRequest = axios.create({
  baseURL: `${import.meta.env.PROD ? 'https' : 'http'}://${API_URL}`,
  withCredentials: true,
});

type MayBeError<T> = (T & { error?: true }) | { error: true };

const shortUrl = async (url: string): Promise<MayBeError<ShortUrlResponse>> => { 
  const res = await apiRequest.post(`/url`, {url}).catch(() => null);
  if (!res) return { error: true };

  return res.data;
};

export default { shortUrl };
