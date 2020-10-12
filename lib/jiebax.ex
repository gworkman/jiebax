defmodule Jiebax do
  use Rustler, otp_app: :jiebax, crate: :jieba_nif

  def cut(_sentence), do: error()

  def cut_all(_sentence), do: error()

  def cut_for_search(_sentence), do: error()

  def tokenize(_sentence), do: error()

  def error(), do: :erlang.nif_error(:nif_not_loaded)
end
