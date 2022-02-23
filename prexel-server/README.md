# Prexel-Server

A restful API server using the `prexel` library.

## URL
https://neociber-prexel.herokuapp.com/

## Endpoints

### Evaluate an expression

- **URL**:
  - `/eval`
  

- **Method**:
  - `POST`
  

- **Query Parameters**:
  - `only_result`:
    - `true`: If the result is successful the result will be a plain number otherwise a json object with an error.
    - `false`: The result will always be a json.
    

- **Request Object:**

  | Key | Type | Required | Description |
  | --- | --- | --- | --- |
  | expression | `string` | `required` | The expression to evaluate |
  | type | `string` |  `optional` | Numeric type to use: `decimal`, `float`, `integer` or `complex`. Default is `decimal` |
  | variables | `object` | `optional` | Variables to use in the expression |

- **Response Object:**

  | Key | Type | Required | Description |
  | --- | --- | --- | --- |
  | result | `string` | `required` | The result of the expression |
  | error | `string` or `null` | `required` | The error if any |

### Example
- **Request:**

  ```json5
  // POST: https://neociber-prexel.herokuapp.com/
  {
    "expression": "(x - y) ^ 2",
    "type": "float",
    "variables": {
      "x": 10,
      "y": 3.5
    }
  }
  ```
  
- **Response:**

  ```json
  {
    "result": "42.25",
    "error": null
  }
  ```
