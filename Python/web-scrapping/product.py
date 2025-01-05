""""Product class to represent a product in a store."""
class Product:
    """"Product class to represent a product in a store."""
    def __init__(self, name, price, description, review):
        # Inicializar los datos del producto
        self._init()

        #Comprobar los valores de los atributos
        if isinstance(name, str) is False:
            print("Product::__init__ - Name must be a string")
            return
        if isinstance(price, float) is False or price < 0.0:
            print("Product::__init__ - Price must be a float")
            return
        if isinstance(description, str) is False:
            print("Product::__init__ - Description must be a string")
            return
        if isinstance(review, int) is False or review < 0:
            print("Product::__init__ - Review must be an int")
            return

        # Asignar los valores a los atributos
        self._valid       = True
        self._name        = name
        self._price       = price
        self._description = description
        self._review      = review

    def _init(self):
        self._name        = ""
        self._price       = 0.0
        self._description = ''
        self._review      = 0
        self._valid       = False

    def __str__(self):
        return f'Name -> {self._name}\nPrice -> {self._price}\nDescription -> {self._description}\nReview -> {self._review}'
    
    def get_name(self):
        """Return the name of the product."""
        return self._name
     
    def get_price(self):
        """Return the price of the product."""
        return self._price
    
    def get_description(self):
        """Return the description of the product."""
        return self._description
    
    def get_review(self):
        """Return the review of the product."""
        return self._review
    
    def is_valid(self):
        """Return the valid of the product."""
        return self._valid

