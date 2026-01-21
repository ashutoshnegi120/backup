defmodule Secrets do
  def secret_add(secret) do
    fn pre -> pre + secret end
  end

  def secret_subtract(secret) do
    fn pre -> pre - secret end
  end

  def secret_multiply(secret) do
     fn pre -> pre * secret end
  end

  def secret_divide(secret) do
    fn pre -> trunc(pre / secret) end
  end

  def secret_and(secret) do
    fn pre -> Bitwise.band(pre,secret) end
  end

  def secret_xor(secret) do
    fn pre -> Bitwise.bxor(pre,secret) end
  end

  def secret_combine(secret_function1, secret_function2) do
    fn pre -> pre 
    |> secret_function1.()
    |>secret_function2.()
    end
  end
end
