export const config = {
  runtime: 'edge',
};

export default async function handler(req) {
  const API_KEY = '7ed63c331cfb0a7ef831a95a';
  const url = new URL(req.url);
  const from = url.searchParams.get('from');
  const to = url.searchParams.get('to');
  const amount = parseFloat(url.searchParams.get('amount'));

  if (!from || !to || isNaN(amount)) {
    return new Response(
      JSON.stringify({
        error: 'Missing or invalid parameters. Required: from, to, amount',
      }),
      {
        status: 400,
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
        },
      }
    );
  }

  try {
    const response = await fetch(
      `https://v6.exchangerate-api.com/v6/${API_KEY}/latest/${from}`
    );
    const data = await response.json();
    const rate = data.conversion_rates[to];

    if (!rate) {
      return new Response(
        JSON.stringify({ error: 'Target currency not found' }),
        {
          status: 400,
          headers: {
            'Content-Type': 'application/json',
            'Access-Control-Allow-Origin': '*',
          },
        }
      );
    }

    const result = amount * rate;
    const timestamp = new Date().toISOString();

    return new Response(
      JSON.stringify({
        from,
        to,
        amount,
        result,
        rate,
        timestamp,
      }),
      {
        status: 200,
        headers: {
          'Content-Type': 'application/json',
          'Access-Control-Allow-Origin': '*',
          'Access-Control-Allow-Methods': 'GET, OPTIONS',
          'Access-Control-Allow-Headers': 'Content-Type',
        },
      }
    );
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