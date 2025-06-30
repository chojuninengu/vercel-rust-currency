export const config = {
  runtime: 'edge',
};

export default async function handler(req) {
  const API_KEY = '7ed63c331cfb0a7ef831a95a';
  const url = `https://v6.exchangerate-api.com/v6/${API_KEY}/latest/USD`;

  try {
    const response = await fetch(url);
    const data = await response.json();
    const currencies = Object.keys(data.conversion_rates).sort();

    return new Response(JSON.stringify(currencies), {
      status: 200,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
        'Access-Control-Allow-Methods': 'GET, OPTIONS',
        'Access-Control-Allow-Headers': 'Content-Type',
      },
    });
  } catch (error) {
    return new Response(JSON.stringify({ error: error.message }), {
      status: 500,
      headers: {
        'Content-Type': 'application/json',
        'Access-Control-Allow-Origin': '*',
      },
    });
  }
} 