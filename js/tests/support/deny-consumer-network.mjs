// Loaded before the installed package so the fixture detects network attempts.
import dns from 'node:dns';
import http from 'node:http';
import https from 'node:https';
import net from 'node:net';
import tls from 'node:tls';

globalThis.__issue195NetworkAttempts = [];
const deny = (name) => function denied() {
  globalThis.__issue195NetworkAttempts.push(name);
  throw new Error(`network access is disabled for the offline consumer: ${name}`);
};
net.Socket.prototype.connect = deny('net.Socket.connect');
net.connect = deny('net.connect');
net.createConnection = deny('net.createConnection');
tls.connect = deny('tls.connect');
http.request = deny('http.request');
http.get = deny('http.get');
https.request = deny('https.request');
https.get = deny('https.get');
dns.lookup = deny('dns.lookup');
dns.resolve = deny('dns.resolve');
dns.promises.lookup = deny('dns.promises.lookup');
dns.promises.resolve = deny('dns.promises.resolve');
globalThis.fetch = deny('fetch');
