export async function fetchItem(id) {
  return await (await fetch(`/items/${id}`)).json();
}

export async function checkOut(id) {
  return await (
    await fetch(`/items/${id}/check_out`, {
      method: "POST",
    })
  ).json();
}

export async function checkIn(id) {
  return await (
    await fetch(`/items/${id}/check_in`, {
      method: "POST",
    })
  ).json();
}

export async function updateTag(id, k, v) {
  return await (
    await fetch(`/items/${id}/tags/${k}`, { method: "PUT", body: v })
  ).json();
}

export async function deleteTag(id, k) {
  return await (
    await fetch(`/items/${id}/tags/${k}`, { method: "DELETE" })
  ).json();
}
