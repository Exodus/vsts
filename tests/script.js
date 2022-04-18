import http from 'k6/http';
import { check }  from 'k6';
import { sleep } from 'k6';
export const options = {
  vus: 200,
  duration: '30s',
};

const urls = {
  generate: "http://localhost:3030/gen",
  validate: "http://localhost:3030/validate/",
  link: "http://localhost:3030/auth"
}

export default function () {
  let res = http.get(urls.generate);
  sleep(1);
  check(res, {
    '200: JWT Generated': (r) => r.status === 200,
  });

  let jwt = res.body;

  res = http.get(urls.validate + jwt)
  check(res, {
    '200: JWT Validated': (r) => r.status === 200,
  });

  res = http.get(
    urls.link,
    {
      headers: {
        'X-FORWARDED-Uri': 'http://localhost/link/' + jwt,
      }
    }
  );
  check(res, {
    '200: JWT Link Validated': (r) => r.status === 200,
  });
}