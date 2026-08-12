import type { Metadata } from 'next';

export const siteName = 'Drift CLI';
export const siteUrl = 'https://drift-cli.greyharborsoftware.com';
export const repositoryUrl = 'https://github.com/cuzz22000/drift-cli';
export const siteDescription =
  'Tenant-scoped administration and recovery for Drift, with explicit commands for operators and automation.';
export const siteKeywords = [
  'Drift CLI',
  'Drift administration',
  'tenant administration',
  'API key management',
  'soft-delete recovery',
  'Rust CLI',
  'operator automation',
  'JSON command output',
] as const;

export const socialCard = {
  url: '/brand/social-card.png',
  width: 1731,
  height: 909,
  alt: 'Drift CLI tenant-scoped administration and recovery',
} as const;

function withTrailingSlash(path: string): string {
  if (path === '/') {
    return path;
  }

  return path.endsWith('/') ? path : `${path}/`;
}

export function buildPageMetadata({
  title,
  description,
  canonicalPath,
}: {
  title: string;
  description: string | undefined;
  canonicalPath: string;
}): Metadata {
  const canonical = withTrailingSlash(canonicalPath);
  const resolvedDescription = description ?? siteDescription;

  return {
    title,
    description: resolvedDescription,
    alternates: {
      canonical,
    },
    openGraph: {
      title,
      description: resolvedDescription,
      url: canonical,
      siteName,
      type: 'website',
      images: [socialCard],
    },
    twitter: {
      card: 'summary_large_image',
      title,
      description: resolvedDescription,
      images: [socialCard.url],
    },
  };
}
