"""Functions to manage a users shopping cart items."""

from collections.abc import Iterable


def add_item(current_cart: dict, items_to_add: Iterable) -> dict:
    """Add items to shopping cart.

    Arguments:
    ---------
        current_cart (dict): The current shopping cart.
        items_to_add (iterable): The items to add to the cart.

    Returns:
    -------
        dict: The updated user cart dictionary.

    """
    for item in items_to_add:
        current_cart[item] = current_cart.setdefault(item, 0) + 1
    return current_cart


def read_notes(notes: Iterable) -> dict:
    """Create user cart from an iterable notes entry.

    Arguments:
    ---------
        notes (iterable): Group of items to add to cart.

    Returns:
    -------
        dict: A user shopping cart dictionary.

    """
    return add_item({}, notes)


def update_recipes(ideas: dict, recipe_updates: Iterable) -> dict:
    """Update the recipe ideas dictionary.

    Arguments:
    ---------
        ideas (dict): The "recipe ideas" dict.
        recipe_updates (iterable): Updates for the ideas section.

    Returns:
    -------
        dict: The updated "recipe ideas" dict.

    """
    updated_recipes = {}
    updated_recipes |= ideas
    updated_recipes |= recipe_updates
    return updated_recipes


def sort_entries(cart: dict) -> dict:
    """Sort a user's shopping cart in alphabetical order.

    Arguments:
    ---------
        cart (dict): A user's shopping cart dictionary.

    Returns:
    -------
        dict: A user's shopping cart sorted in alphabetical order.

    """
    return sorted(cart.items())


def send_to_store(cart: dict, aisle_mapping: dict) -> dict:
    """Combine user's order to aisle and refrigeration information.

    Arguments:
    ---------
        cart (dict): The user's shopping cart dictionary.
        aisle_mapping (dict): The aisle and refrigeration information dictionary.

    Returns:
    -------
        dict: The fulfillment dictionary ready to send to store.

    """
    return sorted(
        {key: [cart[key], *aisle_mapping[key]] for key in cart}.items(), reverse=True
    )


def update_store_inventory(fulfillment_cart: dict, store_inventory: dict) -> dict:
    """Update store inventory levels with user order.

    Arguments:
    ---------
        fulfillment_cart (dict): The fulfillment cart to send to store.
        store_inventory (dict): The stores available inventory.

    Returns:
    -------
        dict: The store_inventory updated.

    """
    for key, value in fulfillment_cart.items():
        store_inventory[key][0] = store_inventory.setdefault(key, 0)[0] - value[0]
        if store_inventory[key][0] <= 0:
            store_inventory[key][0] = "Out of Stock"
    return store_inventory
