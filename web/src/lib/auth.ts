type AuthenticatedUser = {
  id: number;
  email: string;
};
export async function getAuthenticatedUser() {
  const response = await fetch("/api/users");
  if (!response.ok) {
    return null;
  }
  return (await response.json()) as AuthenticatedUser;
}
