import 'server-only';
import { getSessionFromHeaders } from '@konobangu/auth/server';
import { env } from '@konobangu/env';
import { Svix } from 'svix';

export const send = async (eventType: string, payload: object) => {
  if (!env.SVIX_TOKEN) {
    throw new Error('SVIX_TOKEN is not set');
  }

  const svix = new Svix(env.SVIX_TOKEN);
  const session = await getSessionFromHeaders();
  const { orgId } = session;

  if (!orgId) {
    return;
  }

  return svix.message.create(orgId, {
    eventType,
    payload: {
      eventType,
      ...payload,
    },
    application: {
      name: orgId,
      uid: orgId,
    },
  });
};

export const getAppPortal = async () => {
  if (!env.SVIX_TOKEN) {
    throw new Error('SVIX_TOKEN is not set');
  }

  const svix = new Svix(env.SVIX_TOKEN);
  const session = await getSessionFromHeaders();
  const { orgId } = session;

  if (!orgId) {
    return;
  }

  return svix.authentication.appPortalAccess(orgId, {
    application: {
      name: orgId,
      uid: orgId,
    },
  });
};