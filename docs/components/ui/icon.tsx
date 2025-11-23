import type { LucideIcon } from 'lucide-react';

export function create({ icon: Icon }: { icon: LucideIcon }): React.ReactElement {
  return <Icon />;
}
