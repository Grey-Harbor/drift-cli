import type { Metadata } from 'next';
import Link from 'next/link';

import { SiteFooter } from '@/components/site-footer';
import {
  buildPageMetadata,
  repositoryUrl,
  siteDescription,
  siteName,
  siteUrl,
  socialCard,
} from '@/lib/seo';

export const metadata: Metadata = buildPageMetadata({
  title: 'Tenant-scoped administration and recovery for Drift',
  description: siteDescription,
  canonicalPath: '/',
});

const capabilities = [
  {
    title: 'Tenant-scoped keys',
    description:
      'List, create, revoke, and rotate credentials inside the tenant established by the supplied admin key.',
  },
  {
    title: 'Known-ID recovery',
    description:
      'Inspect and restore soft-deleted vertices or edges while preserving Drift version checks.',
  },
  {
    title: 'Automation-ready output',
    description:
      'Use stable JSON envelopes, documented exit codes, and credential-safe configuration in scripts and CI.',
  },
] as const;

const paths = [
  {
    title: 'Tutorial',
    description: 'Build the CLI and administer an existing Drift tenant from the ground up.',
    href: '/docs/tutorial',
  },
  {
    title: 'How-to',
    description: 'Configure access, manage keys, recover records, and use JSON output.',
    href: '/docs/how-to',
  },
  {
    title: 'Explanation',
    description: 'Understand client boundaries, credentials, and Drift tenant administration.',
    href: '/docs/explanation',
  },
  {
    title: 'Reference',
    description: 'Check exact commands, configuration, environment variables, and exit codes.',
    href: '/docs/reference',
  },
] as const;

const operatorContexts = [
  {
    title: 'Local terminals',
    description: 'Replace hand-built administrative requests with explicit, discoverable commands.',
  },
  {
    title: 'CI automation',
    description: 'Use machine-readable output and coarse exit codes without placing secrets in arguments.',
  },
  {
    title: 'Operational runbooks',
    description: 'Keep sensitive key and recovery actions reviewable, repeatable, and tenant-bound.',
  },
] as const;

const structuredData = {
  '@context': 'https://schema.org',
  '@type': 'SoftwareApplication',
  name: siteName,
  applicationCategory: 'DeveloperApplication',
  applicationSubCategory: 'Command-line administration client',
  operatingSystem: 'Linux, macOS, and Windows',
  softwareRequirements: 'Rust 1.85 or newer when building from source',
  description: siteDescription,
  url: siteUrl,
  codeRepository: repositoryUrl,
  image: new URL(socialCard.url, siteUrl).toString(),
  isPartOf: {
    '@type': 'SoftwareApplication',
    name: 'Drift',
    url: 'https://drift.greyharborsoftware.com',
  },
};

export default function HomePage() {
  return (
    <>
      <script
        type="application/ld+json"
        dangerouslySetInnerHTML={{ __html: JSON.stringify(structuredData) }}
      />
      <main className="landing" id="main">
        <section className="hero">
          <div className="hero-copy">
            <span className="eyebrow">Operator tooling for Drift</span>
            <h1>Drift CLI</h1>
            <p className="lede">
              Tenant-scoped administration and recovery through explicit commands—without
              hand-built JSON, fragile curl workflows, or shortcuts around Drift authorization.
            </p>
            <div className="actions">
              <Link className="button primary" href="/docs/tutorial">
                Start with the tutorial
              </Link>
              <Link className="button secondary" href="/docs/reference/commands">
                See the commands
              </Link>
            </div>
          </div>

          <aside className="hero-panel" aria-label="What Drift CLI gives you">
            <div className="hero-brand">
              <img
                src="/drift-cli-mark.svg"
                alt="Drift CLI current and terminal prompt mark"
                width={176}
                height={176}
              />
              <div>
                <span className="eyebrow">A focused companion</span>
                <strong>Operate the API you already trust.</strong>
              </div>
            </div>
            {capabilities.map((capability) => (
              <div className="hero-panel-card" key={capability.title}>
                <strong>{capability.title}</strong>
                <p>{capability.description}</p>
              </div>
            ))}
          </aside>
        </section>

        <section className="section" aria-labelledby="principles-heading">
          <div className="section-heading">
            <p className="eyebrow">Focused by design</p>
            <h2 id="principles-heading">Administrative work, kept inside the boundary</h2>
            <p>
              Drift remains authoritative for behavior, authorization, tenancy, and persistence.
              The CLI makes its supported operator workflows easier to invoke and automate.
            </p>
          </div>
          <div className="card-grid">
            {capabilities.map((capability) => (
              <article className="info-card" key={capability.title}>
                <h3>{capability.title}</h3>
                <p>{capability.description}</p>
              </article>
            ))}
          </div>
        </section>

        <section className="section" aria-labelledby="why-heading">
          <div className="section-heading">
            <p className="eyebrow">Why it exists</p>
            <h2 id="why-heading">An HTTP client, not a privileged backdoor</h2>
            <p>
              Drift CLI replaces ad hoc administrative requests while preserving the same API
              contracts, tenant context, key scopes, and optimistic-concurrency rules as every
              other Drift client.
            </p>
          </div>
        </section>

        <section className="section" aria-labelledby="docs-heading">
          <div className="section-heading">
            <p className="eyebrow">Guides &amp; reference</p>
            <h2 id="docs-heading">Choose your path</h2>
            <p>Learn a workflow, solve a task, inspect a contract, or understand a boundary.</p>
          </div>
          <div className="path-grid">
            {paths.map((path) => (
              <article className="path-card" key={path.title}>
                <h3>{path.title}</h3>
                <p>{path.description}</p>
                <Link href={path.href}>Open {path.title.toLowerCase()}</Link>
              </article>
            ))}
          </div>
        </section>

        <section className="section" aria-labelledby="contexts-heading">
          <div className="section-heading">
            <p className="eyebrow">Where it fits</p>
            <h2 id="contexts-heading">The places operators already work</h2>
          </div>
          <div className="story-grid">
            {operatorContexts.map((context) => (
              <article className="story-card" key={context.title}>
                <h3>{context.title}</h3>
                <p>{context.description}</p>
              </article>
            ))}
          </div>
        </section>

        <SiteFooter />
      </main>
    </>
  );
}
